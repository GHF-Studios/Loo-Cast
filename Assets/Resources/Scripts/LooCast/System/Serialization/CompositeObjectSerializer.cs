using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public abstract class CompositeObjectSerializer<SerializableType> : Serializer, ICompositeObjectSerializer
    {
        #region Constructors
        public CompositeObjectSerializer() : base(SerializationType.CompositeObject, typeof(SerializableType), typeof(XElement))
        {

        }
        #endregion

        #region Methods
        public object Serialize(string name, object serializableCompositeObject) => Serialize(name, serializableCompositeObject);
        public object Deserialize(object serializedCompositeObject) => Deserialize(serializedCompositeObject);

        public abstract XElement Serialize(string name, SerializableType serializableCompositeObject);
        public abstract SerializableType Deserialize(XElement serializedCompositeObject);
        #endregion
    }
}
