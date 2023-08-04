using System;
using System.Collections.Generic;

namespace LooCast.System.Serialization
{
    public class SerializableObjectTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializableObjectType { get; private set; }
        public HashSet<Type> SubSerializablePrimitiveTypes { get; private set; }
        public HashSet<SerializableObjectTypeInfo> SubSerializableObjectTypes { get; private set; }
        #endregion

        #region Constructors
        public SerializableObjectTypeInfo(Type serializableObjectType, HashSet<Type> subSerializablePrimitiveTypes, HashSet<SerializableObjectTypeInfo> subSerializableObjectTypes)
        {
            SerializableObjectType = serializableObjectType;
            SubSerializablePrimitiveTypes = subSerializablePrimitiveTypes;
            SubSerializableObjectTypes = subSerializableObjectTypes;
        }
        #endregion
    }
}
