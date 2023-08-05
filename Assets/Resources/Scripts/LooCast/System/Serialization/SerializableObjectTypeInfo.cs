using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableObjectTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableObjectType { get; private set; }
        public HashSet<SerializablePrimitiveTypeInfo> SubSerializablePrimitiveTypeInfos { get; private set; }
        public HashSet<SerializableObjectTypeInfo> SubSerializableObjectTypeInfos { get; private set; }
        #endregion

        #region Constructors
        public SerializableObjectTypeInfo(Type serializableObjectType, HashSet<SerializablePrimitiveTypeInfo> subSerializablePrimitiveTypeInfos, HashSet<SerializableObjectTypeInfo> subSerializableObjectTypeInfos)
        {
            SerializableObjectType = serializableObjectType;
            SubSerializablePrimitiveTypeInfos = subSerializablePrimitiveTypeInfos;
            SubSerializableObjectTypeInfos = subSerializableObjectTypeInfos;
        }
        #endregion
    }
}
