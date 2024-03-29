"""
Custom Selenium Scrapper Script for Generating Problems and Tests from LeetCode Links
"""
import sys
import re
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options
from selenium import webdriver


def driver_setup(flag):
    option = Options()
    option.headless = flag
    # Hardcoded firefox location
    option.binary_location = "/opt/firefox/firefox-bin"
    return webdriver.Firefox(options=option)


def rust_setup(driver):
    button = driver.find_element(By.CSS_SELECTOR, "button .flex")
    button.click()
    driver.implicitly_wait(2)
    button.find_element(By.XPATH, "./../..").find_element(
        By.TAG_NAME, "ul"
    ).find_elements(By.TAG_NAME, "li")[12].click()


def get_func(driver):
    func = "class Solution"
    while "class Solution" in func:
        driver.implicitly_wait(5)
        rust_setup(driver)
        driver.implicitly_wait(5)
        func = (
            driver.find_element(By.CLASS_NAME, "mtk10")
            .find_element(By.XPATH, "./../../..")
            .text
        )
    return func


def get_title_etc(driver):
    driver.implicitly_wait(5)
    try:
        simple_title = driver.find_elements(By.CLASS_NAME, "mr-2.text-lg")[0]
    except IndexError:
        try:
            simple_title = driver.find_elements(By.CLASS_NAME, "mr-2.text-xl")[0]
        except IndexError:
            print("No Title Was Found on the Webpage")
            exit()
    difc = driver.find_element(By.CLASS_NAME, "mt-3").find_elements(By.TAG_NAME, "div")[
        0
    ]
    return (simple_title.text, int(simple_title.text.partition(".")[0]), difc.text)


def get_examples(driver):
    good_exms = [
        [
            s.replace("[", "vec![")
            for s in re.findall(
                # For matching strings, lists, bools, and numbers
                r'\[[\[(-?\d+,?)(".*?")(true|false)\]]*\]|-?\d+,?|".*?"|true|false',
                ex.text.partition("Explanation")[0],
            )
        ]
        for ex in driver.find_elements(By.TAG_NAME, "pre")
    ]
    outputs = [x.pop() for x in good_exms]
    return (good_exms, outputs)


def make_func(func):
    return (
        " ".join(
            [
                line
                for line in (func).splitlines()
                if line.strip() != "impl Solution {"
                and line != "}"
                and "//" not in line
            ]
        )
        .strip()
        .removesuffix("}")
        .strip()
        + "\n    todo!() \n}"
    )


def make_examples(func, inputs, outputs):
    return "\n".join(
        [
            "fn main() {",
            " " * 4 + "group_print!(",
            " " * 8
            + func.partition("(")[0].removeprefix("pub fn ").strip()
            + ","
            + (" and apply String::from," if '"' in inputs[0][0] else ""),
            ",\n".join(
                [
                    " " * 8 + ", ".join(x) + "; " + outputs[i]
                    for (i, x) in enumerate(inputs)
                ]
            ),
            " " * 4 + ");",
            "",
        ]
    )


def make_test(func, prob_num, inputs, outputs):
    processed_func = func.partition("(")[0].removeprefix("pub fn ").strip()
    initials = "".join([part[0] for part in processed_func.split("_")])
    return "\n".join(
        [
            "#[test]",
            "fn " + initials + "_" + str(prob_num) + "() {",
            " " * 4 + "group_test!(",
            " " * 8
            + processed_func
            + ","
            + (" and apply String::from," if '"' in inputs[0][0] else ""),
            ",\n".join(
                [
                    " " * 8 + ", ".join(x) + "; " + outputs[i]
                    for (i, x) in enumerate(inputs)
                ]
            ),
            " " * 4 + ");",
            "}",
        ]
    )


def make_solution(simple_title, difc, func):
    return "\n".join(
        [
            "/// " + " - ".join([simple_title, "`" + difc + "`"]),
            "///",
            "/// # Idea",
            "/// _",
            "///",
            "/// # Conclusion",
            "/// _",
            func,
        ]
    )


def write_nums(lines, patt, w_num):
    for line in lines:
        mch = re.match(patt, line)
        if mch is None:
            continue
        num = int(mch.group(1))
        if num > w_num:
            return line
    return None


def write_solution(solution_insert, prob_num):
    """
    Open solutions file,
    Find the lowest problem number higher than scrapped problem,
    Prepend the function before it in the file,
    Otherwise append to the end
    """
    (content, saved) = ("", "")
    with open("./src/solutions/mod.rs", "r", encoding="utf-8") as file:
        content = file.read()
        saved = write_nums(content.splitlines(), r"/// (\d+)\.", prob_num)

    with open("./src/solutions/mod.rs", "w", encoding="utf-8") as file:
        if saved is not None:
            file.write(content.replace(saved, "\n\n".join([solution_insert, saved])))
        else:
            file.write("\n\n".join([content, solution_insert]))


def write_test(test_insert, prob_num):
    (content, saved) = ("", "")
    with open("./src/main.rs", "r", encoding="utf-8") as file:
        content = file.read()
        saved = write_nums(content.splitlines(), r"fn \w+_(\d+)", prob_num)

    with open("./src/main.rs", "w", encoding="utf-8") as file:
        if saved is not None:
            saved = "#[test]\n" + saved
            file.write(content.replace(saved, "\n\n".join([test_insert, saved])))
        else:
            file.write("\n\n".join([content, test_insert]))


def write_examples(test_insert):
    # FIX: Indentation breaks if this is not separated
    par = "{"
    content = ""
    with open("./src/main.rs", "r", encoding="utf-8") as file:
        content = file.read().replace("fn main() " + par, test_insert)
    with open("./src/main.rs", "w", encoding="utf-8") as file:
        file.write(content)


def write_readme(simple_title, difc):
    content = ""
    with open("./README.md", "r", encoding="utf-8") as file:
        content = file.read()
        # For Easy and Medium, we can abuse guaranteed double new lines,
        # Hard is the end of file, so we can just consume everything
        mch = re.search(r"(?s)### " + difc + r"(\n\n.*?\n\n|.*)", content)
        if mch is None:
            return -1
        lines = mch.group().splitlines()
        line = write_nums(lines, r"- \[.\] (\d+).", int(simple_title.partition(".")[0]))
        (old, new) = ("", "")
        if line is not None:
            (old, new) = (line, "- [ ] " + simple_title + "\n" + line)
        else:
            (old, new) = (
                lines[len(lines) - 2],
                lines[len(lines) - 2] + "\n" + "- [ ] " + simple_title,
            )
        content = content.replace(mch.group(), mch.group().replace(old, new))
    with open("./README.md", "w", encoding="utf-8") as file:
        file.write(content)
    return 0


if __name__ == "__main__":
    # Load page from arguments,
    web_driver = driver_setup(True)
    try:
        web_driver.get(sys.argv[1])
    except IndexError:
        print("Please enter an address to scrap!")
        web_driver.quit()
        exit()
    # Scrap important details like title, function, and examples,
    (title, prob_number, difficulty) = get_title_etc(web_driver)
    (simple_inputs, simple_outputs) = get_examples(web_driver)
    # Process them into proper code
    pure_func = make_func(get_func(web_driver))
    solution = make_solution(title, difficulty, pure_func)
    example = make_examples(pure_func, simple_inputs, simple_outputs)
    test = make_test(pure_func, prob_number, simple_inputs, simple_outputs)
    # Write them to files.
    write_solution(solution, prob_number)
    write_test(test, prob_number)
    write_examples(example)
    if write_readme(title, difficulty) == -1:
        print("README not updated due to unexpected design")
    web_driver.quit()
