echo off
dir /b /s C:\Users\1\Downloads\NAP_74\*.* > list1.txt
for /f "tokens=*" %%X in (list1.txt) do cscript.exe //nologo D:\Rust\Nap_convert\Doc2Pdf\SaveAsPDF.js "%%X"
del list1.txt
