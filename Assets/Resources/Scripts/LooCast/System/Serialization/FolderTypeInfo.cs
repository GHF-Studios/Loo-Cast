using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public sealed class FolderTypeInfo : TypeInfo
    {
        #region Delegates
        public delegate void Serialize(string folderName, string parentFolderPath, object folder, out DirectoryInfo serializedFolder);
        public delegate void Deserialize(DirectoryInfo serializedFolder, out object folder);
        #endregion
        
        #region Properties
        public PropertyInfo[] Properties { get; set; }
        public FieldInfo[] Fields { get; set; }

        public HashSet<FileTypeInfo> FileTypeDependencies { get; set; }
        public HashSet<FolderTypeInfo> FolderTypeDependencies { get; set; }

        public bool OverrideSerialization { get; set; }
        public bool OverrideDeserialization { get; set; }

        public Serialize SerializeDelegate { get; set; }
        public Deserialize DeserializeDelegate { get; set; }
        #endregion

        #region Constructors
        public FolderTypeInfo(Type type) : base(type, Serializability.Folder)
        {
        }
        #endregion
    }
}
