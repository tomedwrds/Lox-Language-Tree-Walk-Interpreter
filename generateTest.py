import os
import re
base_dir = 'src/tests/'
test_dir = "variable"
dir = base_dir + test_dir
directory = os.fsencode(dir)

with open(dir + "/mod.rs", "w") as testFile:
    #add in start of file boiler plate
    testFile.writelines(["#[cfg(test)]\n", "mod tests {\n", "\tuse crate::tests::run_from_file;\n\n"])
    
    for file in os.listdir(directory):
        filename = file.decode()
        if filename.endswith(".lox"):
            file_path = dir + "/" + filename
            with open(file_path, "r") as testCase:
                #test specific boiler plate
                testFile.writelines(["\t#[test]\n", "\tfn " +test_dir +"_" + filename.split(".")[0]  + "() {\n"])
                fileString = testCase.read()
                expectedOutput = re.findall('expect: (.+?)\n', fileString)
                
                expectedCompileError = None
                expectedRuntimeError = None
                line = 1
                for testLine in fileString.split('\n'):
                    expectedCompileError = re.search('Error at (.*)', testLine)
                    if expectedCompileError:
                        break
                    expectedRuntimeError = re.search('expect runtime error (.*)', testLine)
                    if expectedRuntimeError:
                        break
                    line += 1
                
                if expectedOutput:
                    testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), {expectedOutput});\n".replace("'", '"'))             
                elif expectedCompileError:
                    token, message = expectedCompileError.group(1).split(': ')
                    testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), [\"Line {line}] Error at {token}\", \"Error message: {message}\"]);\n")
                elif expectedRuntimeError:
                    type, message = expectedRuntimeError.group(1).split(': ')
                    testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), [\"Line {line}] Runtime {type} Error\", \"Error message: {message}\"]);\n")
                else:
                    print(f"Invalid test format! {file_path}")
                    
                testFile.write("\t}\n\n")
                    
        else:
            continue
        
    #add in end of file boiler plate
    testFile.writelines(["}"])
    