using System;
using UnityEngine;

namespace LooCast.System.Paths
{
    [Serializable]
    public struct FolderPath : IFolderPath
    {
        #region Properties
        public string GUSP { get; private set; }

        public string HierarchyFolderName => hierarchyFolderName;
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyFolderName;
        #endregion

        #region Constructors
#nullable enable
        public FolderPath(string hierarchyFolderName, FolderPath parentHierarchyFolderPath)
        {
            if (!IsValidHierarchyFolderName(hierarchyFolderName))
            {
                throw new ArgumentException($"Invalid hierarchy folder name: {hierarchyFolderName}");
            }
            
            GUSP = parentHierarchyFolderPath == null ? $"{hierarchyFolderName}" : $"{parentHierarchyFolderPath}/{hierarchyFolderName}";
            
            this.hierarchyFolderName = hierarchyFolderName;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusp, out FolderPath? hierarchyFolderPath)
        {
            hierarchyFolderPath = null;

            string[] parts = gusp.Split(new char[] { '/' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length == 0)
            {
                return false;
            }

            string hierarchyFolderName = parts.Last();
            
            if (!IsValidHierarchyFolderName(hierarchyFolderName))
            {
                return false;
            }

            if (parts.Length == 1)
            {
                hierarchyFolderPath = new FolderPath(hierarchyFolderName, null);
                return true;
            }

            string parentHierarchyFolderPathString = string.Join("/", parts.Take(parts.Length - 1));

            if (!TryParse(parentHierarchyFolderPathString, out FolderPath? parentHierarchyFolderPath))
            {
                return false;
            }

            hierarchyFolderPath = new FolderPath(hierarchyFolderName, parentHierarchyFolderPath!);
            return true;
        }
#nullable disable

        private static bool IsValidHierarchyFolderName(string hierarchyFolderName)
        {
            if (string.IsNullOrEmpty(hierarchyFolderName) || string.IsNullOrWhiteSpace(hierarchyFolderName))
            {
                return false;
            }

            foreach (char character in hierarchyFolderName)
            {
                if (!char.IsLetterOrDigit(character) && character != '_')
                {
                    return false;
                }
            }

            return true;
        }
        #endregion

        #region Methods

        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSP;
        }

        public override bool Equals(object obj)
        {
            if (obj is FolderPath)
            {
                return Equals((FolderPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(FolderPath otherHierarchyFolderPath)
        {
            return otherHierarchyFolderPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FolderPath hierarchyFolderPath1, FolderPath hierarchyFolderPath2)
        {
            return hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static bool operator !=(FolderPath hierarchyFolderPath1, FolderPath hierarchyFolderPath2)
        {
            return !hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static implicit operator string(FolderPath hierarchyFolderPath)
        {
            return hierarchyFolderPath.GUSP;
        }

#nullable enable
        public static implicit operator FolderPath?(string gusp)
        {
            if (TryParse(gusp, out FolderPath? hierarchyFolderPath))
            {
                return hierarchyFolderPath;
            }
            else
            {
                throw new ArgumentException($"The string '{gusp}' is not a valid Namespace GUSP.");
            }
        }
#nullable disable
        #endregion
    }
}
