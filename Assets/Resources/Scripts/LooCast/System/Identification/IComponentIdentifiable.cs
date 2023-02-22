namespace LooCast.System.Identification
{
    public interface IComponentIdentifiable : IGameObjectIdentifiable
    {
        #region Properties
        IComponentIdentifier ComponentIdentifier { get; }
        #endregion
    }
}
