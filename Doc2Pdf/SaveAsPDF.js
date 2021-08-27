var obj = new ActiveXObject("Scripting.FileSystemObject");
var docPath = WScript.Arguments(0);
docPath = obj.GetAbsolutePathName(docPath);

var pdfPath = null;
if(docPath.indexOf("rtf") !== -1){
	pdfPath = docPath.replace(/\.rtf[^.]*$/, ".pdf");
} else if(docPath.indexOf("doc") !== -1){
	pdfPath = docPath.replace(/\.doc[^.]*$/, ".pdf");
}

if(pdfPath !== null){
var objWord = null;

	try
	{
		objWord = new ActiveXObject("Word.Application");
		objWord.Visible = false;
		var objDoc = objWord.Documents.Open(docPath);
		var format = 17;
		objDoc.SaveAs(pdfPath, format);
		objDoc.Close();
		WScript.Echo("Saving '" + docPath + "' as '" + pdfPath + "'...");
	}
	finally
	{
		if (objWord != null)
		{
			objWord.Quit();
		}
	}
}
