using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public class HierarchyPath
    {
        #region Properties
        public string[] PathSubStrings => pathSubStrings;
        public bool IsRoot => PathSubStrings.Length == 1;
        public string PathString => pathString;
        public HierarchyPath ParentPath => parentPath;
        #endregion

        #region Fields
        [SerializeField] private readonly string[] pathSubStrings;
        private readonly string pathString;
        private readonly HierarchyPath parentPath;
        #endregion

        #region Constructors
        public HierarchyPath(string[] pathSubStrings)
        {
            this.pathSubStrings = pathSubStrings;
            pathString = string.Join('/', PathSubStrings);
            parentPath = new HierarchyPath(PathSubStrings.Take(PathSubStrings.Length - 1).ToArray());
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string pathString, out HierarchyPath? hierarchyPath)
        {
            hierarchyPath = null;

            string[] pathSubStrings = pathString.Split(new char[] { '/' }, StringSplitOptions.RemoveEmptyEntries);

            if (pathSubStrings.Length == 0)
            {
                return false;
            }

            hierarchyPath = new HierarchyPath(pathSubStrings);
            return true;
        }
#nullable disable
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

#nullable enable
        public static implicit operator HierarchyPath?(string hierarchyPathString)
        {
            if (TryParse(hierarchyPathString, out HierarchyPath? hierarchyPath))
            {
                return hierarchyPath;
            }
            else
            {
                throw new ArgumentException($"The string '{hierarchyPathString}' is not a valid HierarchyPath.");
            }
        }
#nullable disable
        #endregion
    }
}
