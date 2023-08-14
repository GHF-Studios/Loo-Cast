using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class PrimitiveTypeInfo : TypeInfo
    {
        #region Delegates
        public delegate void Serialize(string primitiveName, object primitive, out XAttribute serializedPrimitive);
        public delegate void Deserialize(XAttribute serializedPrimitive, out object primitive);
        #endregion

        #region Properties
        public Serialize SerializeDelegate { get; private set; }
        public Deserialize DeserializeDelegate { get; private set; }
        #endregion

        #region Constructors
        public PrimitiveTypeInfo(Type type, Serialize serializeDelegate, Deserialize deserializeDelegate) : base(type, Serializability.Primitive)
        {
            SerializeDelegate = serializeDelegate;
            DeserializeDelegate = deserializeDelegate;
        }
        #endregion
    }
}
