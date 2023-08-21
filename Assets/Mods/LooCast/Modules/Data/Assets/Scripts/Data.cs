﻿using System;
using System.IO;
using UnityEngine;

namespace LooCast.Data
{
    using Core;
    
    [Serializable]
    public static class Data
    {
        #region Static Properties
        public static string Path
        {
            get
            {
                return System.IO.Path.Combine(Application.dataPath, "Data");
            }
        }
        #endregion

        #region Static Methods
        public static void ResetAll()
        {
            string path = $"{Application.dataPath}/.Data";
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
        #endregion
    }
}
