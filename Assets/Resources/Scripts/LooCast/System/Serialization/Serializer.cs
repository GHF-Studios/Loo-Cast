using System;

namespace LooCast.System.Serialization
{
    using LooCast.System.Paths;
    
    public abstract class Serializer
    {
        #region Properties
        public SerializationType SerializationType { get; private set; }
        public Type SerializableType { get; private set; }
        public Type SerializedType { get; private set; }
        #endregion

        #region Constructors
        public Serializer(SerializationType serializationType, Type serializableType, Type serializedType)
        {
            SerializationType = serializationType;
            SerializableType = serializableType;
            SerializedType = serializedType;
        }
        #endregion
    }
}
