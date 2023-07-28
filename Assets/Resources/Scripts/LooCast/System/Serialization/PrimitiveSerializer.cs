using System;

namespace LooCast.System.Serialization
{
    using LooCast.System.Paths;

    public abstract class PrimitiveSerializer<SerializableType, SerializedType> : Serializer
    {
        #region Constructors
        public PrimitiveSerializer() : base(typeof(SerializableType), typeof(SerializedType))
        {
        }
        #endregion

        #region Methods
        public abstract SerializedType Serialize(SerializableType serializablePrimitive);
        public abstract SerializableType Deserialize(SerializedType serializedPrimitive);
        #endregion
    }
}
