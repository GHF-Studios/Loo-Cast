using System.IO;

namespace LooCast.System.Serialization
{
    public abstract class CompositeFileSerializer<SerializableType> : Serializer, ICompositeFileSerializer
    {
        #region Constructors
        public CompositeFileSerializer() : base(SerializationType.CompositeFile, typeof(SerializableType), typeof(FileInfo))
        {
            
        }
        #endregion

        #region Methods
        public object Serialize(string name, object serializableCompositeFile) => Serialize(name, serializableCompositeFile);
        public object Deserialize(object serializedCompositeFile) => Deserialize(serializedCompositeFile);

        public abstract FileInfo Serialize(string name, SerializableType serializableCompositeFile);
        public abstract SerializableType Deserialize(FileInfo serializedCompositeFile);
        #endregion
    }
}
