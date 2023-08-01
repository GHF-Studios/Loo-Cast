using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface IPrimitiveAttributeSerializer
    {
        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute);
        #endregion
    }
    
    public interface IPrimitiveAttributeSerializer<PrimitiveAttributeType> : IPrimitiveAttributeSerializer
    {
        #region Methods
        public void Serialize(string primitiveAttributeName, PrimitiveAttributeType primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public void Deserialize(XAttribute serializedPrimitiveAttribute, out PrimitiveAttributeType primitiveAttribute);
        #endregion
    }
}
