using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public abstract class HierarchyPath
    {
        #region Properties
        public string PathString { get; }
        public abstract HierarchyPath ParentPath { get; }
        #endregion

        #region Constructors
        protected HierarchyPath(string pathString)
        {
            PathString = pathString;
        }
        #endregion

        #region Static Methods
        public static bool TryParse(string pathString, out HierarchyPath hierarchyPath)
        {
            if (HierarchyObjectPath.TryParse(pathString, out var objectPath))
            {
                hierarchyPath = objectPath;
                return true;
            }
            
            if (HierarchyFilePath.TryParse(pathString, out var filePath))
            {
                hierarchyPath = filePath;
                return true;
            }
            
            if (HierarchyFolderPath.TryParse(pathString, out var folderPath))
            {
                hierarchyPath = folderPath;
                return true;
            }

            hierarchyPath = null;
            return false;
        }
        #endregion
    }
}
