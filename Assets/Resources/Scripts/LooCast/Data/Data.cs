using System;
using System.IO;
using UnityEngine;

namespace LooCast.Data
{
    public class Data
    {
        public static string Path
        {
            get
            {
                return $"{Application.dataPath}/.Data/";
            }
        }

        public static void ResetAll()
        {
            string path = $"{Application.dataPath}/.Data";
            // create path if not exists
            if (!Directory.Exists(path))
            {
                Directory.CreateDirectory(path);
            }
            string[] filePaths = Directory.GetFiles(path);
            string[] directoryPaths = Directory.GetDirectories(path);
            
            foreach (string filePath in filePaths)
            {
                string name = new FileInfo(filePath).Name;
                if (name != "Default.meta")
                {
                    File.Delete(filePath);
                }
            }

            foreach (string directoryPath in directoryPaths)
            {
                string name = new DirectoryInfo(directoryPath).Name;
                if (name != "Default")
                {
                    Directory.Delete(directoryPath, true);
                }
            }
        }
    }
}
