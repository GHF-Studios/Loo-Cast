using System;
using System.Reflection;
using System.Linq;

namespace LooCast.System.Serialization
{
    public abstract class SerializableTypeMetaInfo
    {
        #region Properties
        public Type SerializableType { get; private set; }
        public SerializableTypeMetaInfoType SerializableTypeMetaInfoType { get; private set; }
        public PropertyInfo[] Properties { get; private set; }
        public FieldInfo[] Fields { get; private set; }
        public Type[] UniqueSubTypes { get; private set; }
        public bool IsSerializationCompletelyOverridden { get; private set; }
        public bool IsSerializableTypeInfoCachingOverridden { get; private set; }
        #endregion

        #region Constructors
        protected SerializableTypeMetaInfo(SerializableTypeMetaInfoType serializableTypeMetaInfoType, Type serializableType, PropertyInfo[] properties, FieldInfo[] fields, bool isSerializationCompletelyOverridden, bool isSerializableTypeInfoCachingOverridden)
        {
            SerializableTypeMetaInfoType = serializableTypeMetaInfoType;
            SerializableType = serializableType;
            Properties = properties;
            Fields = fields;
            UniqueSubTypes = Properties.Select(property => property.PropertyType).Concat(Fields.Select(field => field.FieldType)).Distinct().ToArray();
            IsSerializationCompletelyOverridden = isSerializationCompletelyOverridden;
            IsSerializableTypeInfoCachingOverridden = isSerializableTypeInfoCachingOverridden;
        }
        #endregion
    }
}
