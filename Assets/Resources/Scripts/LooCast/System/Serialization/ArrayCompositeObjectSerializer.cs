using System;
using System.Linq;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ArrayCompositeObjectSerializer<ArrayType> : CompositeObjectSerializer<ArrayType[]>
    {
        #region Properties
        public Type CompositeObjectArrayType { get; private set; }
        public ICompositeObjectSerializer ArrayTypeCompositeObjectSerializer { get; private set; }
        #endregion

        #region Constructors
        public ArrayCompositeObjectSerializer() : base()
        {
            CompositeObjectArrayType = typeof(ArrayType);
            if (!SerializationManager.Instance.IsPrimitiveType(CompositeObjectArrayType))
            {
                throw new ArgumentException($"Type '{CompositeObjectArrayType}' is not a composite object type!");
            }
            ArrayTypeCompositeObjectSerializer = SerializationManager.Instance.GetCompositeObjectSerializer(CompositeObjectArrayType);
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, ArrayType[] serializableCompositeObjects)
        {
            XElement serializedCompositeObjectArray = new XElement(name);
            for (int i = 0; i < serializableCompositeObjects.Length; i++)
            {
                serializedCompositeObjectArray.Add((XElement)ArrayTypeCompositeObjectSerializer.Serialize("Item", serializableCompositeObjects[i]));
            }
            return serializedCompositeObjectArray;
        }

        public override ArrayType[] Deserialize(XElement serializedCompositeObjectArray)
        {
            XElement[] serializedCompositeObjects = serializedCompositeObjectArray.Elements("Item").ToArray();
            ArrayType[] serializableCompositeObjects = new ArrayType[serializedCompositeObjects.Length];
            for (int i = 0; i < serializedCompositeObjects.Length; i++)
            {
                serializableCompositeObjects[i] = (ArrayType)ArrayTypeCompositeObjectSerializer.Deserialize(serializedCompositeObjects[i]);
            }
            return serializableCompositeObjects;
        }
        #endregion
    }
}
