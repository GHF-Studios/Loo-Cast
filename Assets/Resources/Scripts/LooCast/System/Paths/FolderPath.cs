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
        public bool IsRoot
        {
            get
            {
                return FolderNames.Count == 0;
            }
        }
        public string GUSP { get; private set; }
        public bool IsRelative => isRelative;
        public PathType PathType => PathType.Folder;
        public List<string> FolderNames => folderNames;
        /// <summary>
        /// If possible, cache the result of this, as it cannot be cached inside the FolderPath struct and thus has to be computed every time it is accessed!
        /// </summary>
        public FolderPath ParentFolderPath
        {
            get
            {
                if (folderNames.Count == 0 && isRelative)
                {
                    return (FolderPath)string.Empty;
                }
                else if (folderNames.Count == 0 && !isRelative)
                {
                    return (FolderPath)"/";
                }
                else
                {
                    StringBuilder guspBuilder = new StringBuilder();

                    if (!IsRelative)
                    {
                        guspBuilder.Append("/");
                    }

                    guspBuilder.Append(string.Join("/", folderNames.Take(folderNames.Count - 1)));
                    
                    return (FolderPath)guspBuilder.ToString();
                }
            }
        }
        /// <summary>
        /// If possible, cache the result of this, as it cannot be cached inside the FolderPath struct and thus has to be computed every time it is accessed!
        /// </summary>
        public string FolderName
        {
            get
            {
                if (folderNames.Count == 0 && isRelative)
                {
                    return string.Empty;
                }
                else if (folderNames.Count == 0 && !isRelative)
                {
                    return "Root";
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
            this.folderNames = new List<string>(folderNames);

            StringBuilder guspBuilder = new StringBuilder();

            if (!isRelative)
            {
                guspBuilder.Append("/");
            }

            guspBuilder.Append(string.Join("/", folderNames));

            GUSP = guspBuilder.ToString();
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string folderGUSP, out FolderPath? folderPath)
        {
            if (folderGUSP == "/")
            {
                folderPath = new FolderPath(false, Array.Empty<string>());
                return true;
            }
            
            folderPath = null;

            bool isRelative = folderGUSP[0] != '/';

            string[] folderNames = folderGUSP.Split('/', StringSplitOptions.RemoveEmptyEntries);

            if (folderNames == null || (folderNames.Length == 0 && isRelative) || folderNames.Any(folderName => !StringUtil.IsAlphaNumeric(folderName)))
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

        public static explicit operator FolderPath(string gusp)
        {
            if (TryParse(gusp, out FolderPath? folderPath))
            {
                return (FolderPath)folderPath!;
            }
            else
            {
                throw new ArgumentException($"The string '{gusp}' is not a valid Folder GUSP.");
            }
        }

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
