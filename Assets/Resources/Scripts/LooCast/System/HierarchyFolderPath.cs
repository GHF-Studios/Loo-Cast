using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public class HierarchyFolderPath : HierarchyElementPath
    {
        #region Properties
        public string HierarchyFolderName => hierarchyFolderName;
#nullable enable
        public HierarchyFolderPath? ParentHierarchyFolderPath => parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyFolderName;
#nullable enable
        [SerializeField] private readonly HierarchyFolderPath? parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public HierarchyFolderPath(string hierarchyFolderName, HierarchyFolderPath? parentHierarchyFolderPath, string? gusp = null) : base(gusp == null ? parentHierarchyFolderPath == null ? $"{hierarchyFolderName}" : $"{parentHierarchyFolderPath}/{hierarchyFolderName}" : gusp)
        {
            if (!IsValidHierarchyFolderName(hierarchyFolderName))
            {
                throw new ArgumentException($"Invalid hierarchy folder name: {hierarchyFolderName}");
            }
            this.hierarchyFolderName = hierarchyFolderName;
            this.parentHierarchyFolderPath = parentHierarchyFolderPath;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusp, out HierarchyFolderPath? hierarchyFolderPath)
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
                hierarchyFolderPath = new HierarchyFolderPath(hierarchyFolderName, null);
                return true;
            }

            string parentHierarchyFolderPathString = string.Join("/", parts.Take(parts.Length - 1));

            if (!TryParse(parentHierarchyFolderPathString, out HierarchyFolderPath? parentHierarchyFolderPath))
            {
                return false;
            }

            hierarchyFolderPath = new HierarchyFolderPath(hierarchyFolderName, parentHierarchyFolderPath!);
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

        #region Overrides
        public override string ToString()
        {
            return GUSP;
        }

        public override bool Equals(object obj)
        {
            if (obj is HierarchyFolderPath)
            {
                return Equals((HierarchyFolderPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(HierarchyFolderPath otherHierarchyFolderPath)
        {
            return otherHierarchyFolderPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(HierarchyFolderPath hierarchyFolderPath1, HierarchyFolderPath hierarchyFolderPath2)
        {
            return hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static bool operator !=(HierarchyFolderPath hierarchyFolderPath1, HierarchyFolderPath hierarchyFolderPath2)
        {
            return !hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static implicit operator string(HierarchyFolderPath hierarchyFolderPath)
        {
            return hierarchyFolderPath.GUSP;
        }

#nullable enable
        public static implicit operator HierarchyFolderPath?(string gusp)
        {
            if (TryParse(gusp, out HierarchyFolderPath? hierarchyFolderPath))
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
