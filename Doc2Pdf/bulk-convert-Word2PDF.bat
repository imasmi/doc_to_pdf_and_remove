echo off
for %%X in (C:\Users\1\Downloads\NAP_191\*.docx) do cscript.exe //nologo D:\Rust\Nap_convert\Doc2Pdf\SaveAsPDF.js "%%X"
for %%X in (C:\Users\1\Downloads\NAP_191\*.doc) do cscript.exe //nologo D:\Rust\Nap_convert\Doc2Pdf\SaveAsPDF.js "%%X"
