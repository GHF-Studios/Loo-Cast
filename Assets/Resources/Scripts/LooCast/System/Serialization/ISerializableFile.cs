using System.IO;

namespace LooCast.System.Serialization
{
    public interface ISerializableFile
    {
        #region Methods
        public void Serialize(string fileName, string fileExtension, out FileInfo serializedFile);
        public void Deserialize(FileInfo serializedFile);
        #endregion
    }
}
