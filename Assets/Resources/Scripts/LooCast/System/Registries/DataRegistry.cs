namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;

    public sealed class DataRegistry : Registry<IDataIdentifier, IData>
    {
        #region Constructors
        public DataRegistry() : base(MainManager.Instance.MainRegistry) 
        { 
            
        }
        #endregion
    }
}
