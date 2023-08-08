using System;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public class SerializableFolderTypeMetaInfo : SerializableTypeMetaInfo
    {
        #region Constructors
        public SerializableFolderTypeMetaInfo(Type serializableType, PropertyInfo[] properties, FieldInfo[] fields, bool isSerializationCompletelyOverridden, bool isSerializableTypeInfoCachingOverridden) : base(SerializableTypeMetaInfoType.Folder, serializableType, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden)
        {
        }
        #endregion
    }
}
