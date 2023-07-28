using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    using LooCast.System.Paths;

    public abstract class PrimitiveSerializer<SerializableType> : Serializer, IPrimitiveSerializer
    {
        #region Constructors
        public PrimitiveSerializer() : base(SerializationType.Primitive, typeof(SerializableType), typeof(XElement))
        {
        }
        #endregion

        #region Methods
        public object Serialize(string name, object serializablePrimitive) => Serialize(name, serializablePrimitive);
        public object Deserialize(object serializedPrimitive) => Deserialize(serializedPrimitive);
        
        public abstract XElement Serialize(string name, SerializableType serializablePrimitive);
        public abstract SerializableType Deserialize(XElement serializedPrimitive);
        #endregion
    }
}
