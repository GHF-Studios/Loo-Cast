using System;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ArrayPrimitiveSerializer<ArrayType> : PrimitiveSerializer<ArrayType[]>
    {
        #region Properties
        public Type PrimitiveArrayType { get; private set; }
        public IPrimitiveSerializer ArrayTypePrimitiveSerializer { get; private set; }
        #endregion

        #region Constructors
        public ArrayPrimitiveSerializer() : base()
        {
            PrimitiveArrayType = typeof(ArrayType);
            if (!SerializationManager.Instance.IsPrimitiveType(PrimitiveArrayType))
            {
                throw new ArgumentException($"Type '{PrimitiveArrayType}' is not a primitive type!");
            }
            ArrayTypePrimitiveSerializer = SerializationManager.Instance.GetPrimitiveSerializer(PrimitiveArrayType);
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, ArrayType[] serializablePrimitives)
        {
            XElement serializedPrimitiveArray = new XElement(name);
            for (int i = 0; i < serializablePrimitives.Length; i++)
            {
                serializedPrimitiveArray.Add((XElement)ArrayTypePrimitiveSerializer.Serialize("Item", serializablePrimitives[i]));
            }
            return serializedPrimitiveArray;
        }

        public override ArrayType[] Deserialize(XElement serializedPrimitiveArray)
        {
            XElement[] serializedPrimitives = serializedPrimitiveArray.Elements("Item").ToArray();
            ArrayType[] serializablePrimitives = new ArrayType[serializedPrimitives.Length];
            for (int i = 0; i < serializedPrimitives.Length; i++)
            {
                serializablePrimitives[i] = (ArrayType)ArrayTypePrimitiveSerializer.Deserialize(serializedPrimitives[i]);
            }
            return serializablePrimitives;
        }
        #endregion
    }
}
