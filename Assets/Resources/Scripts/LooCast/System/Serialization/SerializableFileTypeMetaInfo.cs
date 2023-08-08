using System;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public class SerializableFileTypeMetaInfo : SerializableTypeMetaInfo
    {
        #region Constructors
        public SerializableFileTypeMetaInfo(Type serializableType, PropertyInfo[] properties, FieldInfo[] fields, bool isSerializationCompletelyOverridden, bool isSerializableTypeInfoCachingOverridden) : base(SerializableTypeMetaInfoType.File, serializableType, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden)
        {
        }
        #endregion
    }
}
