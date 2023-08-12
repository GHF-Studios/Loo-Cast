using System;
using System.Collections.Generic;
using System.IO;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public sealed class FileTypeInfo : TypeInfo
    {
        #region Delegates
        public delegate void Serialize(string fileName, string fileExtension, string parentFolderPath, object file, out FileInfo serializedFile);
        public delegate void Deserialize(FileInfo serializedFile, out object file);
        #endregion

        #region Properties
        public PropertyInfo[] Properties { get; set; }
        public FieldInfo[] Fields { get; set; }

        public HashSet<NonGenericObjectTypeInfo> NonGenericObjectTypeDependencies { get; set; }
        public HashSet<GenericObjectTypeInfo> GenericObjectTypeDependencies { get; set; }

        public bool OverrideSerialization { get; set; }
        public bool OverrideDeserialization { get; set; }

        public Serialize SerializeDelegate { get; set; }
        public Deserialize DeserializeDelegate { get; set; }
        #endregion

        #region Constructors
        public FileTypeInfo(Type type) : base(type, Serializability.File)
        {
        }
        #endregion
    }
}
