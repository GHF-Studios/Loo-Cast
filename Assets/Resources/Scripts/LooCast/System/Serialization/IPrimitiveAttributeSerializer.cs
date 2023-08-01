using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface IPrimitiveAttributeSerializer
    {
        #region Properties
        Type PrimitiveAttributeType { get; }
        #endregion

        #region Methods
        public void Serialize(object primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute);
        #endregion
    }
    
    public interface IPrimitiveAttributeSerializer<PrimitiveAttributeType> : IPrimitiveAttributeSerializer
    {
        #region Methods
        public void Serialize(PrimitiveAttributeType primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public void Deserialize(XAttribute serializedPrimitiveAttribute, out PrimitiveAttributeType primitiveAttribute);
        #endregion
    }
}
