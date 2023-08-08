using System;
using System.Reflection;

namespace LooCast.System.Serialization
{
    public class SerializableObjectTypeMetaInfo : SerializableTypeMetaInfo
    {
        #region Constructors
        public SerializableObjectTypeMetaInfo(Type serializableType, PropertyInfo[] properties, FieldInfo[] fields, bool isSerializationCompletelyOverridden, bool isSerializableTypeInfoCachingOverridden) : base(SerializableTypeMetaInfoType.Object, serializableType, properties, fields, isSerializationCompletelyOverridden, isSerializableTypeInfoCachingOverridden)
        {
        }
        #endregion
    }
}
