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
        public string GUSP
        {
            get
            {
                StringBuilder guspBuilder = new StringBuilder();
                guspBuilder.Append(filePathParent);
                guspBuilder.Append(':');
                guspBuilder.Append(string.Join("+", objectNames));

                return guspBuilder.ToString();
            }
        }
        public bool IsRelative => isRelative;
        public List<string> ObjectNames => objectNames;
        public FilePath FilePathParent => filePathParent;
        #endregion

        #region Fields
        [SerializeField] private bool isRelative;
        [SerializeField] private List<string> objectNames;
        [SerializeField] private FilePath filePathParent;
        #endregion

        #region Constructors
        public ObjectPath(bool isRelative, FilePath filePathParent, string[] objectNames)
        {
            this.isRelative = isRelative;
            this.objectNames = objectNames.ToList();
            this.filePathParent = filePathParent;
        }
        #endregion

        #region Static Methods
#nullable enable
        public static bool TryParse(string objectGUSP, out ObjectPath? objectPath)
        {
            objectPath = null;

            bool isRelative = objectGUSP[0] == '/';

            string[] parts = objectGUSP.Split(':', '+');

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
        public static ObjectPath Combine(FilePath filePath, ObjectPath objectPath)
        {

        }
        #endregion

        #region Methods
        public ObjectPath GetParentPath() 
        {
            
        }
        public string GetObjectName() 
        {
            
        }
        public bool Equals(ObjectPath other) 
        {
            
        }
        public bool StartsWith(ObjectPath prefix) 
        {
            
        }
        public bool EndsWith(ObjectPath suffix) 
        {
            
        }
        public bool IsSubPathOf(FilePath basePath) 
        {
            
        }
        public bool IsParentPathOf(ObjectPath childPath) 
        {
            
        }
        public bool IsChildPathOf(FilePath parentPath) 
        {
            
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
