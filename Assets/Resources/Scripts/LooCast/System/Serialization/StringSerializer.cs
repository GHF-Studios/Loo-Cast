using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class StringSerializer : IPrimitiveAttributeSerializer<string>
    {
        #region Static Properties
        public static StringSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new StringSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static StringSerializer instance;
        #endregion

        #region Constructors
        private StringSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (string)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(string primitiveAttributeName, string primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out string primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }
            
            primitiveAttribute = serializedPrimitiveAttribute.Value;
        }
        #endregion
    }
}
