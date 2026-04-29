using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public class HierarchyFilePath : HierarchyElementPath
    {
        #region Properties
        public string HierarchyFileName => hierarchyFileName;
        public string HierarchyFileExtension => hierarchyFileExtension;
#nullable enable
        public HierarchyFolderPath ParentHierarchyFolderPath => parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyFileName;
        [SerializeField] private readonly string hierarchyFileExtension;
#nullable enable
        [SerializeField] private readonly HierarchyFolderPath parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public HierarchyFilePath(string hierarchyFileName, string hierarchyFileExtension, HierarchyFolderPath parentHierarchyFolderPath, string? gusp = null) : base(gusp == null ? $"{parentHierarchyFolderPath}/{hierarchyFileName}.{hierarchyFileExtension}" : gusp)
        {
            if (!IsValidHierarchyFileName(hierarchyFileName))
            {
                throw new ArgumentException($"Invalid hierarchy file name: {hierarchyFileName}");
            }

            if (!IsValidHierarchyFileExtension(hierarchyFileExtension))
            {
                throw new ArgumentException($"Invalid hierarchy file extension: {hierarchyFileExtension}");
            }
            
            this.hierarchyFileName = hierarchyFileName;
            this.parentHierarchyFolderPath = parentHierarchyFolderPath;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string gusp, out HierarchyFilePath? hierarchyFilePath)
        {
            hierarchyFilePath = null;

            string[] parts = gusp.Split(new char[] { '/' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length == 0)
            {
                return false;
            }

            string hierarchyFileNameWithExtension = parts.Last();

            if (string.IsNullOrEmpty(hierarchyFileNameWithExtension))
            {
                return false;
            }

            int lastDotIndex = hierarchyFileNameWithExtension.LastIndexOf('.');

            if (lastDotIndex == -1)
            {
                return false;
            }

            string hierarchyFileName = hierarchyFileNameWithExtension.Substring(0, lastDotIndex);
            string hierarchyFileExtension = hierarchyFileNameWithExtension.Substring(lastDotIndex + 1);

            if (!IsValidHierarchyFileName(hierarchyFileName))
            {
                return false;
            }

            if (!IsValidHierarchyFileExtension(hierarchyFileExtension))
            {
                return false;
            }

            if (parts.Length == 1)
            {
                hierarchyFilePath = new HierarchyFilePath(hierarchyFileName, hierarchyFileExtension, new HierarchyFolderPath(string.Empty, null));
                return true;
            }

            string parentHierarchyFolderPathString = string.Join("/", parts.Take(parts.Length - 1));

            if (!HierarchyFolderPath.TryParse(parentHierarchyFolderPathString, out HierarchyFolderPath? parentHierarchyFolderPath))
            {
                return false;
            }

            hierarchyFilePath = new HierarchyFilePath(hierarchyFileName, hierarchyFileExtension, parentHierarchyFolderPath!);
            return true;
        }
#nullable disable

        private static bool IsValidHierarchyFileName(string hierarchyFolderName)
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

        private static bool IsValidHierarchyFileExtension(string fileExtension)
        {
            if (string.IsNullOrEmpty(fileExtension) || string.IsNullOrWhiteSpace(fileExtension))
            {
                return false;
            }

            foreach (char character in fileExtension)
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
            if (obj is HierarchyFilePath)
            {
                return Equals((HierarchyFilePath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(HierarchyFilePath otherHierarchyFilePath)
        {
            return otherHierarchyFilePath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(HierarchyFilePath hierarchyFilePath1, HierarchyFilePath hierarchyFilePath2)
        {
            return hierarchyFilePath1.Equals(hierarchyFilePath2);
        }

        public static bool operator !=(HierarchyFilePath hierarchyFilePath1, HierarchyFilePath hierarchyFilePath2)
        {
            return !hierarchyFilePath1.Equals(hierarchyFilePath2);
        }

        public static implicit operator string(HierarchyFilePath hierarchyFilePath)
        {
            return hierarchyFilePath.GUSP;
        }

#nullable enable
        public static implicit operator HierarchyFilePath?(string gusp)
        {
            if (TryParse(gusp, out HierarchyFilePath? hierarchyFilePath))
            {
                return hierarchyFilePath;
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
