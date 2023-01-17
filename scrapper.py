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


def change_lang(driver):
    button = driver.find_element(By.CSS_SELECTOR, "button .flex")
    button.click()
    driver.implicitly_wait(2)
    button.find_element(By.XPATH, "./../..").find_element(
        By.TAG_NAME, "ul"
    ).find_elements(By.TAG_NAME, "li")[12].click()


def try_func(driver):
    func = "class Solution"
    while "class Solution" in func:
        driver.implicitly_wait(5)
        change_lang(driver)
        driver.implicitly_wait(5)
        func = (
            driver.find_element(By.CLASS_NAME, "mtk10")
            .find_element(By.XPATH, "./../../..")
            .text
        )
    return func


def get_title_and_num(driver):
    simple_title = driver.find_elements(By.CLASS_NAME, "mr-2.text-lg")[0]
    difc = driver.find_element(By.CLASS_NAME, "mt-3").find_elements(By.TAG_NAME, "div")[
        0
    ]
    return (
        " - ".join([simple_title.text, "`" + difc.text + "`"]),
        int(simple_title.text.partition(".")[0]),
    )


def get_tests(driver):
    good_exms = [
        [
            s.replace("[", "vec![")
            for s in re.compile(r'\[[\[(\d+,*)*\],*]*\]|\d+|".*?"|true|false').findall(
                ex.text.partition("Explanation")[0]
            )
        ]
        for ex in driver.find_elements(By.TAG_NAME, "pre")
    ]
    outputs = [x.pop() for x in good_exms]
    return (good_exms, outputs)


def process_func(func):
    return (
        " ".join(
            [
                line
                for line in (func).splitlines()
                if line.strip() != "impl Solution {" and line != "}"
            ]
        )
        .strip()
        .removesuffix("}")
        .strip()
        + "\n    todo!() \n}"
    )


def process_tests(func, inputs, outputs):
    return "\n".join(
        [
            "fn main() {",
            " " * 4 + "group_print!(",
            " " * 8 + func.partition("(")[0].removeprefix("pub fn ").strip() + ",",
            ",\n".join(
                [
                    " " * 8 + ", ".join(x) + "; " + outputs[i]
                    for (i, x) in enumerate(inputs)
                ]
            ),
            " " * 4 + ");",
        ]
    )


def process_solution(full_title, func):
    return "\n".join(
        [
            "/// " + full_title,
            "///",
            "/// # Idea",
            "/// _",
            "/// # Conclusion",
            "/// _",
            func,
        ]
    )


def write_solution(solution_insert, prob_num):
    """
    Open solutions file,
    Find the lowest problem number higher than scrapped problem,
    Prepend the function before it in the file,
    Otherwise append to the end
    """
    patt = re.compile(r"/// (\d+).")
    content = ""
    saved = ""
    with open("./src/solutions/mod.rs", "r", encoding="utf-8") as file:
        lines = file.readlines()
        for line in lines:
            mch = patt.match(line)
            if mch is None:
                continue
            num = int(mch.group(1))
            if num > prob_num:
                saved = line
                break
        content = "".join(lines)

    with open("./src/solutions/mod.rs", "w", encoding="utf-8") as file:
        if saved != "":
            file.write(content.replace(saved, "\n\n".join([solution_insert, saved])))
        else:
            file.write("\n\n".join([content, solution_insert]))


def write_tests(test_insert):
    content = ""
    with open("./src/main.rs", "r", encoding="utf-8") as file:
        content = file.read().replace("fn main() {", test_insert)
    with open("./src/main.rs", "w", encoding="utf-8") as file:
        file.write(content)


# TODO: Add Support for Adding Things to README.md file
if __name__ == "__main__":
    # Load page from arguments,
    web_driver = driver_setup(True)
    web_driver.get(sys.argv[1])
    # Scrap important details like title, function, and examples,
    (title, prob_number) = get_title_and_num(web_driver)
    (simple_inputs, simple_outputs) = get_tests(web_driver)
    # Process them into proper code
    pure_func = process_func(try_func(web_driver))
    solution = process_solution(title, pure_func)  # pylint: disable=invalid-name
    tests = process_tests(  # pylint: disable=invalid-name
        pure_func, simple_inputs, simple_outputs
    )
    # Write them to files.
    write_solution(solution, prob_number)
    write_tests(tests)
    web_driver.quit()
