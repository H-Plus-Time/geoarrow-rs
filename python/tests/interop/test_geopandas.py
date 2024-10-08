import geoarrow.rust.core as gars
import geodatasets
import geopandas as gpd
from geopandas.testing import assert_geodataframe_equal

nybb_path = geodatasets.get_path("nybb")


# Test it works without error
# Superseded by full assertion when CRS is fixed
def test_geopandas_round_trip():
    gdf = gpd.read_file(nybb_path)
    assert isinstance(gdf, gpd.GeoDataFrame)
    table = gars.from_geopandas(gdf)
    retour = gars.to_geopandas(table)
    assert_geodataframe_equal(gdf, retour)
