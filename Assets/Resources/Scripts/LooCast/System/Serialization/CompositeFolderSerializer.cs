using System.IO;

namespace LooCast.System.Serialization
{
    public abstract class CompositeFolderSerializer<SerializableType> : Serializer, ICompositeFolderSerializer
    {
        #region Constructors
        public CompositeFolderSerializer() : base(SerializationType.CompositeFolder, typeof(SerializableType), typeof(DirectoryInfo))
        {

        }
        #endregion

        #region Methods
        public object Serialize(string name, object serializableCompositeFolder) => Serialize(name, serializableCompositeFolder);
        public object Deserialize(object serializedCompositeFolder) => Deserialize(serializedCompositeFolder);

        public abstract DirectoryInfo Serialize(string name, SerializableType serializableCompositeFolder);
        public abstract SerializableType Deserialize(DirectoryInfo serializedCompositeFolder);
        #endregion
    }
}
