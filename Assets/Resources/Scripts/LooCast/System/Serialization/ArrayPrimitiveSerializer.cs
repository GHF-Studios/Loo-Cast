using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public class ArrayPrimitiveSerializer<ArrayType> : PrimitiveSerializer<ArrayType[]>
    {
        #region Properties
        public Type PrimitiveArrayType { get; private set; }
        public IPrimitiveSerializer ArrayTypePrimitiveSerializer { get; private set; }
        #endregion

        #region Constructors
        public ArrayPrimitiveSerializer() : base()
        {
            PrimitiveArrayType = typeof(ArrayType);
            if (SerializationManager.Instance.IsCompositeType(PrimitiveArrayType))
            {
                throw new ArgumentException($"Type '{PrimitiveArrayType}' is a composite type and cannot be used as a primitive type!");
            }
            ArrayTypePrimitiveSerializer = SerializationManager.Instance.GetPrimitiveSerializer(PrimitiveArrayType);
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, ArrayType[] serializablePrimitives)
        {
            XElement serializedArray = new XElement(name);
            for (int i = 0; i < serializablePrimitives.Length; i++)
            {
                serializedArray.Add((XElement)ArrayTypePrimitiveSerializer.Serialize("Item", serializablePrimitives[i]));
            }
            return serializedArray;
        }

        public override ArrayType[] Deserialize(XElement serializedArray)
        {
            XElement[] serializedPrimitives = serializedArray.Elements("Item").ToArray();
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
