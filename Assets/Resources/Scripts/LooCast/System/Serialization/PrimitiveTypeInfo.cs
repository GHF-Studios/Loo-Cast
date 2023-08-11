using System;
using System.Collections.Generic;
using System.Reflection;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class PrimitiveTypeInfo
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
        public PrimitiveTypeInfo(Serialize serializeDelegate, Deserialize deserializeDelegate)
        {
            SerializeDelegate = serializeDelegate;
            DeserializeDelegate = deserializeDelegate;
        }
        #endregion
    }
}
