using System;
using System.Linq;
using System.Collections.Generic;
using UnityEngine;
using System.Text;

namespace LooCast.System.Paths
{
    [Serializable]
    public struct FolderPath : IFolderPath
    {
        #region Properties
        public string GUSP
        {
            get
            {
                StringBuilder guspBuilder = new StringBuilder();

                if (!IsRelative)
                {
                    guspBuilder.Append("/");
                }

                guspBuilder.Append(string.Join("/", folderNames));

                return guspBuilder.ToString();
            }
        }
        public bool IsRelative => isRelative;
        public List<string> FolderNames => folderNames;
        public FolderPath ParentFolderPath
        {
            get
            {
                if (folderNames.Count == 0)
                {
                    return (FolderPath)string.Empty;
                }
                else
                {
                    return (FolderPath)string.Join("/", folderNames.Take(folderNames.Count - 1));
                }
            }
        }
        public string FolderName
        {
            get
            {
                if (folderNames.Count == 0)
                {
                    return string.Empty;
                }
                else
                {
                    return folderNames[folderNames.Count - 1];
                }
            }
        }
        #endregion

        #region Fields
        [SerializeField] private readonly bool isRelative;
        [SerializeField] private readonly List<string> folderNames;
        #endregion

        #region Constructors
        public FolderPath(bool isRelative, params string[] folderNames)
        {
            if (folderNames == null || folderNames.Length == 0 && isRelative)
            {
                throw new Exception("At least one folder name must be provided when the path is relative!");
            }
            if (folderNames.Any(folderName => !StringUtil.IsAlphaNumeric(folderName)))
            {
                throw new Exception("Folder names must be alphanumeric!");
            }
            
            this.isRelative = isRelative;
            this.folderNames = folderNames.ToList();
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string folderGUSP, out FolderPath? folderPath)
        {
            if (folderGUSP == "/")
            {
                folderPath = new FolderPath(true, Array.Empty<string>());
                return true;
            }
            
            folderPath = null;

            bool isRelative = folderGUSP[0] != '/';

            string[] folderNames = folderGUSP.Split('/');

            if (folderNames == null || folderNames.Length == 0 || folderNames.Any(folderName => !StringUtil.IsAlphaNumeric(folderName)))
            {
                return false;
            }

            folderPath = new FolderPath(isRelative, folderNames);
            return true;
        }
#nullable disable
        #endregion

        #region Methods
        public bool IsChildOf(FolderPath folderPathParent)
        {
            if (folderPathParent.IsRelative && !IsRelative)
            {
                return false;
            }

            if (folderPathParent.FolderNames.Count >= FolderNames.Count)
            {
                return false;
            }
            
            for (int i = 0; i < folderPathParent.FolderNames.Count; i++)
            {
                if (folderPathParent.FolderNames[i] != FolderNames[i])
                {
                    return false;
                }
            }

            return true;
        }

        public bool IsParentOf(FolderPath folderPathChild)
        {
            return folderPathChild.IsChildOf(this);
        }

        public bool IsParentOf(FilePath filePathChild)
        {
            return filePathChild.IsChildOf(this);
        }
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

        public bool Equals(FolderPath otherFolderPath)
        {
            return otherFolderPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FolderPath folderPath1, FolderPath folderPath2)
        {
            return folderPath1.Equals(folderPath2);
        }

        public static bool operator !=(FolderPath folderPath1, FolderPath folderPath2)
        {
            return !folderPath1.Equals(folderPath2);
        }

        public static implicit operator string(FolderPath folderPath)
        {
            return folderPath.GUSP;
        }

#nullable enable
        public static implicit operator FolderPath?(string gusp)
        {
            if (TryParse(gusp, out FolderPath? folderPath))
            {
                return folderPath;
            }
            else
            {
                throw new ArgumentException($"The string '{gusp}' is not a valid Folder GUSP.");
            }
        }
#nullable disable

        public static FolderPath operator +(FolderPath folderPath1, FolderPath folderPath2)
        {
            if (folderPath1.IsRelative && !folderPath2.IsRelative)
            {
                throw new InvalidOperationException("Cannot add an absolute folder path to a relative folder path!");
            }
            else if (!folderPath1.IsRelative && !folderPath2.IsRelative)
            {
                throw new InvalidOperationException("Cannot add two absolute folder paths!");
            }
            else if (!folderPath1.IsRelative && folderPath2.IsRelative)
            {
                return new FolderPath(false, folderPath1.FolderNames.Concat(folderPath2.FolderNames).ToArray());
            }
            else
            {
                return new FolderPath(true, folderPath1.FolderNames.Concat(folderPath2.FolderNames).ToArray());
            }
        }
        
        public static FilePath operator +(FolderPath folderPath, FilePath filePath)
        {
            if (folderPath.IsRelative && !filePath.IsRelative)
            {
                throw new InvalidOperationException("Cannot add an absolute file path to a relative folder path!");
            }
            else if (!folderPath.IsRelative && !filePath.IsRelative)
            {
                throw new InvalidOperationException("Cannot add a file path to an absolute folder path!");
            }
            else if (!folderPath.IsRelative && filePath.IsRelative)
            {
                return new FilePath(false, filePath.FileName, filePath.FileExtension, folderPath);
            }
            else
            {
                return new FilePath(true, filePath.FileName, filePath.FileExtension, folderPath);
            }
        }
        #endregion
    }
}
