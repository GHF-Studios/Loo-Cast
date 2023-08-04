using System;

namespace LooCast.System.Serialization
{
    public class SerializablePrimitiveTypeInfo : SerializableTypeInfo
    {
        #region Properties
        public Type SerializablePrimitiveType { get; private set; }
        #endregion

        #region Constructors
        public SerializablePrimitiveTypeInfo(Type serializablePrimitiveType)
        {
            SerializablePrimitiveType = serializablePrimitiveType;
        }
        #endregion
    }
}
