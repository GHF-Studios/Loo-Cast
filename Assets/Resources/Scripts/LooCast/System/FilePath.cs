using System;
using System.Linq;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public class FilePath : HierarchicalObjectPath
    {
        #region Properties
        public string HierarchyFileName => hierarchyFileName;
        public string HierarchyFileExtension => hierarchyFileExtension;
#nullable enable
        public FolderPath ParentHierarchyFolderPath => parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Fields
        [SerializeField] private readonly string hierarchyFileName;
        [SerializeField] private readonly string hierarchyFileExtension;
#nullable enable
        [SerializeField] private readonly FolderPath parentHierarchyFolderPath;
#nullable disable
        #endregion

        #region Constructors
#nullable enable
        public FilePath(string hierarchyFileName, string hierarchyFileExtension, FolderPath parentHierarchyFolderPath, string? gusp = null) : base(gusp == null ? $"{parentHierarchyFolderPath}/{hierarchyFileName}.{hierarchyFileExtension}" : gusp)
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
        public static bool TryParse(string gusp, out FilePath? hierarchyFilePath)
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
                hierarchyFilePath = new FilePath(hierarchyFileName, hierarchyFileExtension, new FolderPath(string.Empty, null));
                return true;
            }

            string parentHierarchyFolderPathString = string.Join("/", parts.Take(parts.Length - 1));

            if (!FolderPath.TryParse(parentHierarchyFolderPathString, out FolderPath? parentHierarchyFolderPath))
            {
                return false;
            }

            hierarchyFilePath = new FilePath(hierarchyFileName, hierarchyFileExtension, parentHierarchyFolderPath!);
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
            if (obj is FilePath)
            {
                return Equals((FilePath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(FilePath otherHierarchyFilePath)
        {
            return otherHierarchyFilePath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FilePath hierarchyFilePath1, FilePath hierarchyFilePath2)
        {
            return hierarchyFilePath1.Equals(hierarchyFilePath2);
        }

        public static bool operator !=(FilePath hierarchyFilePath1, FilePath hierarchyFilePath2)
        {
            return !hierarchyFilePath1.Equals(hierarchyFilePath2);
        }

        public static implicit operator string(FilePath hierarchyFilePath)
        {
            return hierarchyFilePath.GUSP;
        }

#nullable enable
        public static implicit operator FilePath?(string gusp)
        {
            if (TryParse(gusp, out FilePath? hierarchyFilePath))
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
