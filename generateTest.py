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
                fileString = testCase.read()
                expectedOutput = re.findall('expect: (.+?)\n', fileString)
                if expectedOutput:
                    testFile.writelines(["\t#[test]\n", "\tfn " +test_dir +"_" + filename.split(".")[0]  + "() {\n", f"\t\tassert_eq!(run_from_file(\"{file_path}\"), {expectedOutput});\n".replace("'", '"') , "\t}\n\n"])             
        else:
            continue
        
    #add in end of file boiler plate
    testFile.writelines(["}"])
    