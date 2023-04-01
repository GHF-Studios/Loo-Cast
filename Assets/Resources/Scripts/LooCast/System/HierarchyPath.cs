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
            if (HierarchyFolderPath.TryParse(pathString, out var folderPath))
            {
                hierarchyPath = folderPath;
                return true;
            }

            if (HierarchyFilePath.TryParse(pathString, out var filePath))
            {
                hierarchyPath = filePath;
                return true;
            }

            if (HierarchyObjectPath.TryParse(pathString, out var objectPath))
            {
                hierarchyPath = objectPath;
                return true;
            }

            hierarchyPath = null;
            return false;
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return PathString;
        }

        public override bool Equals(object obj)
        {
            if (obj is HierarchyPath)
            {
                return Equals((HierarchyPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(HierarchyPath otherHierarchyPath)
        {
            return otherHierarchyPath.PathString.Equals(this.PathString);
        }

        public override int GetHashCode()
        {
            return PathString.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(HierarchyPath hierarchyPath1, HierarchyPath hierarchyPath2)
        {
            return hierarchyPath1.Equals(hierarchyPath2);
        }

        public static bool operator !=(HierarchyPath hierarchyPath1, HierarchyPath hierarchyPath2)
        {
            return !hierarchyPath1.Equals(hierarchyPath2);
        }

        public static implicit operator string(HierarchyPath hierarchyPath)
        {
            return hierarchyPath.PathString;
        }

        public static implicit operator HierarchyPath(string pathString)
        {
            if (TryParse(pathString, out var hierarchyPath))
            {
                return hierarchyPath;
            }
            else
            {
                throw new ArgumentException($"The string '{pathString}' is not a valid HierarchyPath.");
            }
        }
        #endregion
    }
}
