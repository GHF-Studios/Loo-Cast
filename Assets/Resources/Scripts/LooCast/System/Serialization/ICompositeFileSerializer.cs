namespace LooCast.System.Serialization
{
    /// <summary>
    /// CAUTION: Composite file serializers should ALWAYS serialize to and deserialize from a FileInfo!
    /// </summary>
    public interface ICompositeFileSerializer : ISerializer
    {
        #region Methods
        object Serialize(string name, object serializableCompositeFile);
        object Deserialize(object serializedCompositeFile);
        #endregion
    }
}
