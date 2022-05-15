package dev.redio.sql;

import com.mysql.cj.jdbc.MysqlDataSource;

import java.sql.Connection;
import java.sql.ResultSet;
import java.sql.ResultSetMetaData;
import java.sql.SQLException;
import java.util.ArrayList;
import java.util.List;

public final class SQL {

    private SQL() {}

    public static Connection openConnection(String user, String password, String hostname, int port, String database)
            throws SQLException {
        var source = new MysqlDataSource();
        source.setUser(user);
        source.setPassword(password);
        source.setServerName(hostname);
        source.setPortNumber(port);
        source.setDatabaseName(database);
        return source.getConnection();
    }

    public static List<String> getColumnTitles(ResultSet resultSet) throws SQLException {
        var titles = new ArrayList<String>();
        ResultSetMetaData metaData = resultSet.getMetaData();
        for (int i = 0; i < metaData.getColumnCount(); i++)
            titles.add(metaData.getCatalogName(i));
        return titles;
    }
    
    public static List<List<Object>> getTableContent(ResultSet resultSet) throws SQLException {
        List<List<Object>> rows = new ArrayList<>();
        ResultSetMetaData metaData = resultSet.getMetaData();
        while (resultSet.next()) {
            var column = new ArrayList<>();
            for (int i = 0; i < metaData.getColumnCount(); i++)
                column.add(resultSet.getObject(i));
            rows.add(column);
        }
        return rows;
    }
}
