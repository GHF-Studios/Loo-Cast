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
        #endregion

        #region Fields
        [SerializeField] private bool isRelative;
        [SerializeField] private List<string> folderNames;
        #endregion

        #region Constructors
        private FolderPath(bool isRelative, params string[] folderNames)
        {
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

        public bool Equals(FolderPath otherHierarchyFolderPath)
        {
            return otherHierarchyFolderPath.GUSP.Equals(this.GUSP);
        }

        public override int GetHashCode()
        {
            return GUSP.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(FolderPath hierarchyFolderPath1, FolderPath hierarchyFolderPath2)
        {
            return hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static bool operator !=(FolderPath hierarchyFolderPath1, FolderPath hierarchyFolderPath2)
        {
            return !hierarchyFolderPath1.Equals(hierarchyFolderPath2);
        }

        public static implicit operator string(FolderPath hierarchyFolderPath)
        {
            return hierarchyFolderPath.GUSP;
        }

#nullable enable
        public static implicit operator FolderPath?(string gusp)
        {
            if (TryParse(gusp, out FolderPath? hierarchyFolderPath))
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
