﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using UnityEngine;

namespace LooCast.System.Paths
{
    [Serializable]
    public struct FilePath : IFilePath
    {
        #region Properties
        public string GUSP
        {
            get
            {
                StringBuilder guspBuilder = new StringBuilder();

                if (folderPathParent != null)
                {
                    guspBuilder.Append($"{folderPathParent}/");
                }
                else if (!isRelative)
                {
                    guspBuilder.Append("/");
                }

                guspBuilder.Append($"{fileName}.{fileExtension}");

                return guspBuilder.ToString();
            }
        }
        public bool IsRelative => isRelative;
        public string FileName => fileName;
        public string FileExtension => fileExtension;
        public FolderPath FolderPathParent => folderPathParent;
        #endregion

        #region Fields
        [SerializeField] private readonly bool isRelative;
        [SerializeField] private readonly string fileName;
        [SerializeField] private readonly string fileExtension;
        [SerializeField] private readonly FolderPath folderPathParent;
        #endregion

        #region Constructors
        public FilePath(bool isRelative, string fileName, string fileExtension, FolderPath folderPathParent)
        {
            if (!StringUtil.IsAlphaNumeric(fileName) || !StringUtil.IsAlphaNumeric(fileExtension))
            {
                throw new Exception("File name and extension must be alphanumeric!");
            }
            if (isRelative && !folderPathParent.IsRelative)
            {
                throw new Exception("Relative file paths must have a relative folder path parent!");
            }
            
            this.isRelative = isRelative;
            this.fileName = fileName;
            this.fileExtension = fileExtension;
            this.folderPathParent = folderPathParent;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string fileGUSP, out FilePath? filePath)
        {
            filePath = null;
            bool isRelative = (fileGUSP[0] != '/');
            string[] parts = fileGUSP.Split(new char[] { '/', '.' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length < 1)
            {
                return false;
            }

            string fileName = parts[parts.Length - 2];
            string fileExtension = parts[parts.Length - 1];

            if (!StringUtil.IsAlphaNumeric(fileName) || !StringUtil.IsAlphaNumeric(fileExtension))
            {
                return false;
            }
            
            FolderPath? folderPathParent;

            if (parts.Length > 1)
            {
                string folderGUSP = string.Join("/", parts.Take(parts.Length - 2));
                if (!isRelative)
                {
                    folderGUSP = "/" + folderGUSP;
                }

                if (!FolderPath.TryParse(folderGUSP, out folderPathParent))
                {
                    return false;
                }
                
                if (isRelative && !((FolderPath)folderPathParent!).IsRelative)
                {
                    return false;
                }
            }
            else
            {
                folderPathParent = null;
            }

            filePath = new FilePath(isRelative, fileName, fileExtension, (FolderPath)folderPathParent!);
            return true;
        }
#nullable disable
        #endregion

        #region Methods
        public bool IsChildOf(FolderPath folderPathParent)
        {
            return this.FolderPathParent == folderPathParent;
        }

        public bool IsParentOf(ObjectPath objectPathChild)
        {
            return objectPathChild.IsChildOf(this);
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

        public bool Equals(FilePath otherFilePath)
        {
            return otherFilePath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FilePath filePath1, FilePath filePath2)
        {
            return filePath1.Equals(filePath2);
        }

        public static bool operator !=(FilePath filePath1, FilePath filePath2)
        {
            return !filePath1.Equals(filePath2);
        }

        public static implicit operator string(FilePath filePath)
        {
            return filePath.GUSP;
        }

        public static explicit operator FilePath(string gusp)
        {
            if (TryParse(gusp, out FilePath? filePath))
            {
                return (FilePath)filePath!;
            }
            else
            {
                throw new ArgumentException($"The string '{gusp}' is not a valid File GUSP.");
            }
        }

        public static ObjectPath operator +(FilePath filePath, ObjectPath objectPath)
        {
            if (filePath.IsRelative && !objectPath.IsRelative)
            {
                throw new Exception("Cannot add an absolute object path to a relative file path!");
            }
            else if (!filePath.IsRelative && !objectPath.IsRelative)
            {
                throw new Exception("Cannot add an absolute object path to an absolute file path!");
            }
            else if (!filePath.IsRelative && objectPath.IsRelative)
            {
                return new ObjectPath(false, filePath, objectPath.ObjectNames.ToArray());
            }
            else
            {
                return new ObjectPath(true, filePath, objectPath.ObjectNames.ToArray());
            }
        }
        #endregion
    }
}