using System;
using UnityEngine;

namespace LooCast.System.Paths
{
    [Serializable]
    public class ObjectPath : IObjectPath
    {
        #region Properties
        public string GUSP { get; private set; }

        public string HierarchyObjectName => hierarchyObjectName;
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyObjectName;
        #endregion

        #region Constructors
#nullable enable
        public ObjectPath(string hierarchyObjectName, FilePath? parentHierarchyFilePath, ObjectPath? parentHierarchyObjectPath)
        {
            if (!IsValidHierarchyObjectName(hierarchyObjectName))
            {
                throw new ArgumentException($"Invalid hierarchy object name: {hierarchyObjectName}");
            }

            if (!IsValidParent(parentHierarchyFilePath, parentHierarchyObjectPath))
            {
                throw new ArgumentException($"An object path is required to have exactly one type of parent path!");
            }

            GUSP = parentHierarchyFilePath == null ? $"{parentHierarchyObjectPath}-{hierarchyObjectName}" : parentHierarchyObjectPath == null ? $"{parentHierarchyFilePath}-{hierarchyObjectName}" : null;
            
            this.hierarchyObjectName = hierarchyObjectName;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusp, out ObjectPath? hierarchyObjectPath)
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

            FilePath? parentHierarchyFilePath = null;
            ObjectPath? parentHierarchyObjectPath = null;

            if (FilePath.TryParse(parentHierarchyElementPathString, out parentHierarchyFilePath))
            {
                hierarchyObjectPath = new ObjectPath(hierarchyObjectName, parentHierarchyFilePath!, null);
                return true;
            }

            if (TryParse(parentHierarchyElementPathString, out parentHierarchyObjectPath))
            {
                hierarchyObjectPath = new ObjectPath(hierarchyObjectName, null, parentHierarchyObjectPath!);
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
        private static bool IsValidParent(FilePath? parentHierarchyFilePath, ObjectPath? parentHierarchyObjectPath)
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

        #region Methods

        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSP;
        }

        public override bool Equals(object obj)
        {
            if (obj is ObjectPath)
            {
                return Equals((ObjectPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(ObjectPath otherHierarchyObjectPath)
        {
            return otherHierarchyObjectPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(ObjectPath hierarchyObjectPath1, ObjectPath hierarchyObjectPath2)
        {
            return hierarchyObjectPath1.Equals(hierarchyObjectPath2);
        }

        public static bool operator !=(ObjectPath hierarchyObjectPath1, ObjectPath hierarchyObjectPath2)
        {
            return !hierarchyObjectPath1.Equals(hierarchyObjectPath2);
        }

        public static implicit operator string(ObjectPath hierarchyObjectPath)
        {
            return hierarchyObjectPath.GUSP;
        }

#nullable enable
        public static implicit operator ObjectPath?(string gusp)
        {
            if (TryParse(gusp, out ObjectPath? hierarchyObjectPath))
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
