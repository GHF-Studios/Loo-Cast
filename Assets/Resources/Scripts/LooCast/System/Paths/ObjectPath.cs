using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using UnityEngine;

namespace LooCast.System.Paths
{
    [Serializable]
    public struct ObjectPath : IObjectPath
    {
        #region Properties
        public string GUSP { get; private set; }
        public bool IsRelative => isRelative;
        public List<string> ObjectNames => objectNames;
        public FilePath FilePathParent => filePathParent;
        /// <summary>
        /// If possible, cache the result of this, as it cannot be cached inside the FolderPath struct and thus has to be computed every time it is accessed!
        /// </summary>
        public ObjectPath ObjectPathParent => new ObjectPath(isRelative, filePathParent, objectNames.Take(objectNames.Count - 1).ToArray());
        /// <summary>
        /// If possible, cache the result of this, as it cannot be cached inside the FolderPath struct and thus has to be computed every time it is accessed!
        /// </summary>
        public string ObjectName
        {
            get
            {
                if (objectNames.Count == 0)
                {
                    return string.Empty;
                }
                else
                {
                    return objectNames[objectNames.Count - 1];
                }
            }
        }
        #endregion

        #region Fields
        [SerializeField] private readonly bool isRelative;
        [SerializeField] private readonly List<string> objectNames;
        [SerializeField] private readonly FilePath filePathParent;
        #endregion

        #region Constructors
        public ObjectPath(bool isRelative, FilePath filePathParent, string[] objectNames)
        {
            if (objectNames == null || objectNames.Length == 0)
            {
                throw new Exception("At least one object name must be provided!");
            }
            if (objectNames.Any(objectName => !StringUtil.IsAlphaNumeric(objectName)))
            {
                throw new Exception("Object names must be alphanumeric!");
            }
            if (isRelative && !filePathParent.IsRelative)
            {
                throw new Exception("Relative object paths must have a relative file path parent!");
            }
            
            this.isRelative = isRelative;
            this.objectNames = objectNames.ToList();
            this.filePathParent = filePathParent;

            StringBuilder guspBuilder = new StringBuilder();
            guspBuilder.Append(filePathParent);
            guspBuilder.Append(':');
            guspBuilder.Append(string.Join("+", objectNames));

            GUSP = guspBuilder.ToString();
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string objectGUSP, out ObjectPath? objectPath)
        {
            objectPath = null;

            bool isRelative = objectGUSP[0] == '/';

            string[] parts = objectGUSP.Split(new char[] { ':', '+' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length < 1)
            {
                return false;
            }

            string[] objectNames = parts.Skip(1).ToArray();

            if (objectNames == null || objectNames.Length == 0 || objectNames.Any(objectName => !StringUtil.IsAlphaNumeric(objectName)))
            {
                return false;
            }

            FilePath? filePathParent;

            if (parts.Length > 1)
            {
                string fileParentGUSP = parts[0];
                if (!isRelative)
                {
                    fileParentGUSP = "/" + fileParentGUSP;
                }
                
                if (!FilePath.TryParse(fileParentGUSP, out filePathParent))
                {
                    return false;
                }
                if (isRelative && !((FilePath)filePathParent!).IsRelative)
                {
                    return false;
                }
            }
            else
            {
                if (!isRelative)
                {
                    return false;
                }
                
                filePathParent = null;
            }

            objectPath = new ObjectPath(isRelative, (FilePath)filePathParent!, objectNames);
            return true;
        }
#nullable disable
        #endregion

        #region Methods
        public bool IsChildOf(FilePath filePathParent)
        {
            return this.FilePathParent == filePathParent;
        }
        
        public bool IsChildOf(ObjectPath objectPathParent)
        {
            if (objectPathParent.IsRelative && !IsRelative)
            {
                return false;
            }

            if (objectPathParent.ObjectNames.Count >= ObjectNames.Count)
            {
                return false;
            }

            for (int i = 0; i < objectPathParent.ObjectNames.Count; i++)
            {
                if (objectPathParent.ObjectNames[i] != ObjectNames[i])
                {
                    return false;
                }
            }

            return true;
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
            if (obj is ObjectPath)
            {
                return Equals((ObjectPath)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(ObjectPath otherObjectPath)
        {
            return otherObjectPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(ObjectPath objectPath1, ObjectPath objectPath2)
        {
            return objectPath1.Equals(objectPath2);
        }

        public static bool operator !=(ObjectPath objectPath1, ObjectPath objectPath2)
        {
            return !objectPath1.Equals(objectPath2);
        }

        public static implicit operator string(ObjectPath objectPath)
        {
            return objectPath.GUSP;
        }

        public static explicit operator ObjectPath(string gusp)
        {
            if (TryParse(gusp, out ObjectPath? objectPath))
            {
                return (ObjectPath)objectPath!;
            }
            else
            {
                throw new ArgumentException($"The string '{gusp}' is not a valid Object GUSP.");
            }
        }

        public static ObjectPath operator +(ObjectPath objectPath1, ObjectPath objectPath2)
        {
            if (objectPath1.IsRelative && !objectPath2.IsRelative)
            {
                throw new Exception("Cannot add a´n absolute object path to a relative object path!");
            }
            else if (!objectPath1.IsRelative && !objectPath2.IsRelative)
            {
                throw new Exception("Cannot add two absolute object paths!");
            }
            else if (!objectPath1.IsRelative && objectPath2.IsRelative)
            {
                return new ObjectPath(false, objectPath1.FilePathParent, objectPath1.ObjectNames.Concat(objectPath2.ObjectNames).ToArray());
            }
            else
            {
                return new ObjectPath(true, objectPath1.FilePathParent, objectPath1.ObjectNames.Concat(objectPath2.ObjectNames).ToArray());
            }
        }
        #endregion
    }
}
