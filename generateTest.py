import os
import re
base_dir = 'src/tests/'
test_dir = "for_loop"
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
                    expectedCompileError = re.findall('Error at (.*) Error at (.*)', testLine)
                    if expectedCompileError:
                        break
                    expectedCompileError = re.findall('Error at (.*)', testLine)
                    
                    if expectedCompileError:
                        break
                    expectedRuntimeError = re.search('expect runtime error (.*)', testLine)
                    if expectedRuntimeError:
                        break
                    line += 1
                
                if expectedOutput:
                    testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), {expectedOutput});\n".replace("'", '"'))             
                elif expectedCompileError:
                    error_message = expectedCompileError[0]
                    if type(error_message) is tuple:
                        error_str = f"\t\tassert_eq!(run_from_file(\"{file_path}\"), ["
                        for i in range(len(error_message)):
                            token, message = error_message[i].split(': ')
                            if i != 0:
                                error_str += ","
                            error_str += f"\"[Line {line}] Error at {token}\", \"Error Message: {message}\""
                        error_str += f"]);\n"
                        testFile.write(error_str)
                    else:
                        token, message = error_message.split(': ')
                        testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), [\"[Line {line}] Error at {token}\", \"Error Message: {message}\"]);\n")
                elif expectedRuntimeError:
                    type, message = expectedRuntimeError.group(1).split(': ')
                    testFile.write(f"\t\tassert_eq!(run_from_file(\"{file_path}\"), [\"[Line {line}] Runtime {type} Error\", \"Error Message: {message}\"]);\n")
                else:
                    print(f"Invalid test format! {file_path}")
                    
                testFile.write("\t}\n\n")
                    
        else:
            continue
        
    #add in end of file boiler plate
    testFile.writelines(["}"])
    