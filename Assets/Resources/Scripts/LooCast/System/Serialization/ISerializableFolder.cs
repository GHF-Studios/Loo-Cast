using System.IO;

namespace LooCast.System.Serialization
{
    public interface ISerializableFolder
    {
        #region Methods
        public void Serialize(string folderName, out DirectoryInfo serializedFolder);
        public void Deserialize(DirectoryInfo serializedFolder);
        #endregion
    }
}
