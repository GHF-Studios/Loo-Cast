namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public sealed class MetaDataRegistry : Registry<IMetaDataIdentifier, IMetaData>
    {
        #region Constructors
        public MetaDataRegistry() : base(MainManager.Instance.MainRegistry)
        {

        }
        #endregion
    }
}
