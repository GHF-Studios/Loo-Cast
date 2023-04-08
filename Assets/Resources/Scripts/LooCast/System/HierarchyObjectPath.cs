using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public class HierarchyObjectPath : HierarchyElementPath
    {
        #region Properties
        public string HierarchyObjectName => hierarchyObjectName;
#nullable enable
        public HierarchyFilePath? ParentHierarchyFilePath => parentHierarchyFilePath;
        public HierarchyObjectPath? ParentHierarchyObjectPath => parentHierarchyObjectPath;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyObjectName;
#nullable enable
        [SerializeField] private readonly HierarchyFilePath? parentHierarchyFilePath;
        [SerializeField] private readonly HierarchyObjectPath? parentHierarchyObjectPath;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public HierarchyObjectPath(string hierarchyObjectName, HierarchyFilePath? parentHierarchyFilePath, HierarchyObjectPath? parentHierarchyObjectPath, string? gusp = null) : base(gusp == null ? (parentHierarchyFilePath == null ? $"{parentHierarchyObjectPath}-{hierarchyObjectName}" : parentHierarchyObjectPath == null ? $"{parentHierarchyFilePath}-{hierarchyObjectName}" : null) : gusp)
        {
            if (!IsValidHierarchyObjectName(hierarchyObjectName))
            {
                throw new ArgumentException($"Invalid hierarchy object name: {hierarchyObjectName}");
            }

            if (!IsValidParent(parentHierarchyFilePath, parentHierarchyObjectPath))
            {
                throw new ArgumentException($"An object path is required to have exactly one type of parent path!");
            }

            this.hierarchyObjectName = hierarchyObjectName;
            this.parentHierarchyFilePath = parentHierarchyFilePath;
            this.parentHierarchyObjectPath = parentHierarchyObjectPath;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusp, out HierarchyObjectPath? hierarchyObjectPath)
        {
            hierarchyObjectPath = null;

            string[] parts = gusp.Split(new char[] { '-' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            string hierarchyObjectName = parts[1];

            if (!IsValidHierarchyObjectName(hierarchyObjectName))
            {
                return false;
            }

            string parentHierarchyElementPathString = parts[0];

            HierarchyFilePath? parentHierarchyFilePath = null;
            HierarchyObjectPath? parentHierarchyObjectPath = null;

            if (HierarchyFilePath.TryParse(parentHierarchyElementPathString, out parentHierarchyFilePath))
            {
                hierarchyObjectPath = new HierarchyObjectPath(hierarchyObjectName, parentHierarchyFilePath!, null);
                return true;
            }

            if (TryParse(parentHierarchyElementPathString, out parentHierarchyObjectPath))
            {
                hierarchyObjectPath = new HierarchyObjectPath(hierarchyObjectName, null, parentHierarchyObjectPath!);
                return true;
            }

            return false;
        }
#nullable disable

        private static bool IsValidHierarchyObjectName(string hierarchyObjectName)
        {
            if (string.IsNullOrEmpty(hierarchyObjectName) || string.IsNullOrWhiteSpace(hierarchyObjectName))
            {
                return false;
            }

            foreach (char character in hierarchyObjectName)
            {
                if (!char.IsLetterOrDigit(character) && character != '_')
                {
                    return false;
                }
            }

            return true;
        }

#nullable enable
        private static bool IsValidParent(HierarchyFilePath? parentHierarchyFilePath, HierarchyObjectPath? parentHierarchyObjectPath)
        {
            if (parentHierarchyFilePath == null && parentHierarchyObjectPath == null)
            {
                return false;
            }

            if (parentHierarchyFilePath != null && parentHierarchyObjectPath != null)
            {
                return false;
            }

            return true;
        }
#nullable disable
        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSP;
        }

        public override bool Equals(object obj)
        {
            if (obj is HierarchyObjectPath)
            {
                return Equals((HierarchyObjectPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(HierarchyObjectPath otherHierarchyObjectPath)
        {
            return otherHierarchyObjectPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(HierarchyObjectPath hierarchyObjectPath1, HierarchyObjectPath hierarchyObjectPath2)
        {
            return hierarchyObjectPath1.Equals(hierarchyObjectPath2);
        }

        public static bool operator !=(HierarchyObjectPath hierarchyObjectPath1, HierarchyObjectPath hierarchyObjectPath2)
        {
            return !hierarchyObjectPath1.Equals(hierarchyObjectPath2);
        }

        public static implicit operator string(HierarchyObjectPath hierarchyObjectPath)
        {
            return hierarchyObjectPath.GUSP;
        }

#nullable enable
        public static implicit operator HierarchyObjectPath?(string gusp)
        {
            if (TryParse(gusp, out HierarchyObjectPath? hierarchyObjectPath))
            {
                return hierarchyObjectPath;
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
