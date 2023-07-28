namespace LooCast.System.Serialization
{
    /// <summary>
    /// CAUTION: Composite folder serializers should ALWAYS serialize to and deserialize from a DirectoryInfo!
    /// </summary>
    public interface ICompositeFolderSerializer : ISerializer
    {
        #region Methods
        object Serialize(string name, object serializableCompositeFolder);
        object Deserialize(object serializedCompositeFolder);
        #endregion
    }
}
